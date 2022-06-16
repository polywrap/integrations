import { namehash, keccak256 } from "./utils";
import { abi, bytecode } from "./contracts/FIFSRegistrar";
import {
  Ethereum_Module,
  Ethereum_TxResponse,
  ConfigureOpenDomainResponse,
  CreateSubdomainInOpenDomainResponse,
  CreateSubdomainInOpenDomainAndSetContentHashResponse,
  TxOverrides,
  RegistrationResult,
  Input_getResolver,
  Input_getExpiryTimes,
  Input_getOwner,
  Input_checkIfRecordExists,
  Input_getAddress,
  Input_getContentHash,
  Input_getReverseResolver,
  Input_getNameFromReverseResolver,
  Input_getNameFromAddress,
  Input_getContentHashFromDomain,
  Input_getAddressFromDomain,
  Input_getTextRecord,
  Input_registerDomain,
  Input_reverseRegisterDomain,
  Input_setAddress,
  Input_setAddressFromDomain,
  Input_setContentHash,
  Input_setContentHashFromDomain,
  Input_setName,
  Input_setOwner,
  Input_setResolver,
  Input_setSubdomainOwner,
  Input_setSubdomainRecord,
  Input_setRecord,
  Input_deployFIFSRegistrar,
  Input_registerSubnodeOwnerWithFIFSRegistrar,
  Input_setTextRecord,
  Input_configureOpenDomain,
  Input_createSubdomainInOpenDomain,
  Input_createSubdomainInOpenDomainAndSetContentHash,
  Input_registerDomainAndSubdomainsRecursively,
  Input_registerSubdomainsRecursively,
} from "./wrap";

export function getResolver(input: Input_getResolver): string {
  const domain = namehash(input.domain);

  const resolverAddress = Ethereum_Module.callContractView({
    address: input.registryAddress,
    method: "function resolver(bytes32 node) external view returns (address)",
    args: [domain],
    connection: input.connection,
  });

  return resolverAddress.unwrap();
}

export function getOwner(input: Input_getOwner): string {
  const owner = Ethereum_Module.callContractView({
    address: input.registryAddress,
    method: "function owner(bytes32 node) external view returns (address)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  return owner.unwrap();
}

export function checkIfRecordExists(input: Input_checkIfRecordExists): bool {
  const recordExists = Ethereum_Module.callContractView({
    address: input.registryAddress,
    method: "function recordExists(bytes32 node) external view returns (address)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  const result = recordExists.unwrap();

  return result == "0x0000000000000000000000000000000000000001";
}

export function getAddress(input: Input_getAddress): string {
  const address = Ethereum_Module.callContractView({
    address: input.resolverAddress,
    method: "function addr(bytes32 node) external view returns (address)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  return address.unwrap();
}

export function getContentHash(input: Input_getContentHash): string {
  const hash = Ethereum_Module.callContractView({
    address: input.resolverAddress,
    method: "function contenthash(bytes32 node) external view returns (bytes)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  return hash.unwrap();
}

export function getAddressFromDomain(
  input: Input_getAddressFromDomain
): string {
  const resolverAddress = getResolver({
    registryAddress: input.registryAddress,
    domain: input.domain,
    connection: input.connection,
  });

  const address = Ethereum_Module.callContractView({
    address: resolverAddress,
    method: "function addr(bytes32 node) external view returns (address)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  return address.unwrap();
}

export function getContentHashFromDomain(
  input: Input_getContentHashFromDomain
): string {
  const resolverAddress = getResolver({
    registryAddress: input.registryAddress,
    domain: input.domain,
    connection: input.connection,
  });

  const hash = Ethereum_Module.callContractView({
    address: resolverAddress,
    method: "function contenthash(bytes32 node) external view returns (bytes)",
    args: [namehash(input.domain)],
    connection: input.connection,
  });

  return hash.unwrap();
}

export function getExpiryTimes(input: Input_getExpiryTimes): string {
  const label = input.domain.split(".")[0];

  const expiryTime = Ethereum_Module.callContractView({
    address: input.registrarAddress,
    method:
      "function expiryTimes(bytes32 label) external view returns (uint256)",
    args: [keccak256(label)],
    connection: input.connection,
  });

  return expiryTime.unwrap();
}

export function getReverseResolver(input: Input_getReverseResolver): string {
  const address = namehash(input.address.substr(2) + ".addr.reverse");

  const resolverAddress = Ethereum_Module.callContractView({
    address: input.registryAddress,
    method: "function resolver(bytes32 node) external view returns (address)",
    args: [address],
    connection: input.connection,
  });

  return resolverAddress.unwrap();
}

export function getNameFromAddress(input: Input_getNameFromAddress): string {
  const address = namehash(input.address.substr(2) + ".addr.reverse");

  const resolverAddress = getReverseResolver({
    registryAddress: input.registryAddress,
    address: input.address,
    connection: input.connection,
  });

  const name = Ethereum_Module.callContractView({
    address: resolverAddress,
    method: "function name(bytes32 node) external view returns (string)",
    args: [address],
    connection: input.connection,
  });

  return name.unwrap();
}

export function getNameFromReverseResolver(
  input: Input_getNameFromReverseResolver
): string {
  const address = namehash(input.address.substr(2) + ".addr.reverse");

  const name = Ethereum_Module.callContractView({
    address: input.resolverAddress,
    method: "function name(bytes32 node) external view returns (string)",
    args: [address],
    connection: input.connection,
  });

  return name.unwrap();
}

export function getTextRecord(input: Input_getTextRecord): string {
  const value = Ethereum_Module.callContractView({
    address: input.resolverAddress,
    method: "function text(bytes32 node, string value) external view returns (string)",
    args: [namehash(input.domain), input.key],
    connection: input.connection,
  });

  return value.unwrap();
}

export function setResolver(input: Input_setResolver): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setResolverTx = Ethereum_Module.callContractMethod({
    address: input.registryAddress,
    method: "function setResolver(bytes32 node, address owner)",
    args: [namehash(input.domain), input.resolverAddress],
    connection: input.connection,
    txOverrides: {
      value: null,
      gasPrice: txOverrides.gasPrice,
      gasLimit: txOverrides.gasLimit,
    },
  });

  return setResolverTx.unwrap();
}

export function registerDomain(
  input: Input_registerDomain
): Ethereum_TxResponse {
  const label = input.domain.split(".")[0];
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const tx = Ethereum_Module.callContractMethod({
    address: input.registrarAddress,
    method: "function register(bytes32 label, address owner)",
    args: [keccak256(label), input.owner],
    connection: input.connection,
    txOverrides: {
      value: null,
      gasPrice: txOverrides.gasPrice,
      gasLimit: txOverrides.gasLimit,
    },
  });

  return tx.unwrap();
}

export function setOwner(input: Input_setOwner): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const tx = Ethereum_Module.callContractMethod({
    address: input.registryAddress,
    method: "function setOwner(bytes32 node, address owner) external",
    args: [namehash(input.domain), input.newOwner],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return tx.unwrap();
}

export function setSubdomainOwner(
  input: Input_setSubdomainOwner
): Ethereum_TxResponse {
  const splitDomain = input.subdomain.split(".");
  const subdomainLabel = splitDomain[0];
  const domain = splitDomain.slice(1, splitDomain.length).join(".");

  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const tx = Ethereum_Module.callContractMethod({
    address: input.registryAddress,
    method:
      "function setSubnodeOwner(bytes32 node, bytes32 label, address owner) external",
    args: [namehash(domain), keccak256(subdomainLabel), input.owner],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return tx.unwrap();
}

export function setSubdomainRecord(
  input: Input_setSubdomainRecord
): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const tx = Ethereum_Module.callContractMethod({
    address: input.registryAddress,
    method:
      "function setSubnodeRecord(bytes32 node, bytes32 label, address owner, address resolver, uint64 ttl)",
    args: [
      namehash(input.domain),
      keccak256(input.label),
      input.owner,
      input.resolverAddress,
      input.ttl,
    ],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return tx.unwrap();
}

export function registerDomainAndSubdomainsRecursively(
  input: Input_registerDomainAndSubdomainsRecursively
): RegistrationResult[] {
  const registrationResults: RegistrationResult[] = []
  const splitDomain = input.domain.split(".");

  if (splitDomain.length < 2) {
    throw new Error("Expected subdomain/domain name. Examples: foo.eth, foo.bar.eth")
  }

  let rootDomain = splitDomain.slice(-1)[0];
  let subdomains = splitDomain.slice(0, -1).reverse();

  for (let i = 0; i < subdomains.length; i++) {
    subdomains[i] = `${subdomains[i]}.${i === 0? rootDomain: subdomains[i - 1]}`
  }

  for (let i = 0; i < subdomains.length; i++) {
    const result: RegistrationResult = {
      name: subdomains[i],
      didRegister: false,
      tx: null
    }

    const isRegistered = checkIfRecordExists({
      registryAddress: input.registryAddress,
      domain: subdomains[i],
      connection: input.connection
    });

    if (!isRegistered) {
      if(i === 0) {
        // Main domain case
        result.tx = registerDomain({
          domain: subdomains[i],
          registrarAddress: input.registrarAddress,
          registryAddress: input.registryAddress,
          owner: input.owner,
          connection: input.connection,
          txOverrides: input.txOverrides
        })

        setResolver({
          domain: subdomains[i],
          registryAddress: input.registryAddress,
          resolverAddress: input.resolverAddress,
          connection: input.connection,
          txOverrides: input.txOverrides
        })

      } else {
        // Subdomain case
        const label = subdomains[i].split('.')[0]
        const domain = subdomains[i].split('.').slice(1).join('.')
  
        result.tx = setSubdomainRecord({
          domain,
          label,
          owner: input.owner,
          resolverAddress: input.resolverAddress,
          ttl: input.ttl,
          registryAddress: input.registryAddress,
          connection: input.connection,
          txOverrides: input.txOverrides
        })
      }

      result.didRegister = true;
    }

    registrationResults.push(result)
  }

  return registrationResults;
}

export function registerSubdomainsRecursively(
  input: Input_registerSubdomainsRecursively
): RegistrationResult[] {
  const registrationResults: RegistrationResult[] = []
  const splitDomain = input.domain.split(".");

  if (splitDomain.length < 3) {
    throw new Error("Expected subdomain name. Example: foo.bar.eth")
  }

  let rootDomain = splitDomain.slice(-2).join(".");
  let subdomains = splitDomain.slice(0, -2).reverse();

  for (let i = 0; i < subdomains.length; i++) {
    subdomains[i] = `${subdomains[i]}.${i === 0? rootDomain: subdomains[i - 1]}`
  }

  for (let i = 0; i < subdomains.length; i++) {
    const result: RegistrationResult = {
      name: subdomains[i],
      didRegister: false,
      tx: null
    }

    const isRegistered = checkIfRecordExists({
      registryAddress: input.registryAddress,
      domain: subdomains[i],
      connection: input.connection
    });

    if (!isRegistered) {
      const label = subdomains[i].split('.')[0]
      const domain = subdomains[i].split('.').slice(1).join('.')

      result.tx = setSubdomainRecord({
        domain,
        label,
        owner: input.owner,
        resolverAddress: input.resolverAddress,
        ttl: input.ttl,
        registryAddress: input.registryAddress,
        connection: input.connection,
        txOverrides: input.txOverrides
      })

      result.didRegister = true;
    }

    registrationResults.push(result)
  }

  return registrationResults;
}

//TODO: Where could this be used on mainnet?
export function setRecord(input: Input_setRecord): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const tx = Ethereum_Module.callContractMethod({
    address: input.registryAddress,
    method:
      "function setRecord(bytes32 node, address owner, address resolver, uint64 ttl)",
    args: [
      namehash(input.domain),
      input.owner,
      input.resolverAddress,
      input.ttl,
    ],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return tx.unwrap();
}

export function setName(input: Input_setName): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setNameTx = Ethereum_Module.callContractMethod({
    address: input.reverseRegistryAddress,
    method: "function setName(string name)",
    args: [input.domain],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return setNameTx.unwrap();
}

export function reverseRegisterDomain(
  input: Input_reverseRegisterDomain
): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  Ethereum_Module.callContractMethod({
    address: input.reverseRegistryAddress,
    method: "function claim(address owner)",
    args: [input.owner],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  const setNameTx = setName({
    reverseRegistryAddress: input.reverseRegistryAddress,
    domain: input.domain,
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
    },
  });

  return setNameTx;
}

export function setAddress(input: Input_setAddress): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setAddrTx = Ethereum_Module.callContractMethod({
    address: input.resolverAddress,
    method: "function setAddr(bytes32 node, address addr)",
    args: [namehash(input.domain), input.address],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return setAddrTx.unwrap();
}

export function setContentHash(
  input: Input_setContentHash
): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setContentHash = Ethereum_Module.callContractMethod({
    address: input.resolverAddress,
    method: "function setContenthash(bytes32 node, bytes hash)",
    args: [namehash(input.domain), input.cid],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return setContentHash.unwrap();
}

export function setAddressFromDomain(
  input: Input_setAddressFromDomain
): Ethereum_TxResponse {
  const resolverAddress = getResolver({
    domain: input.domain,
    registryAddress: input.registryAddress,
    connection: input.connection,
  });

  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setAddrTx = Ethereum_Module.callContractMethod({
    address: resolverAddress,
    method: "function setAddr(bytes32 node, address addr)",
    args: [namehash(input.domain), input.address],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return setAddrTx.unwrap();
}

export function setContentHashFromDomain(
  input: Input_setContentHashFromDomain
): Ethereum_TxResponse {
  const resolverAddress = getResolver({
    domain: input.domain,
    registryAddress: input.registryAddress,
    connection: input.connection,
  });

  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const setContentHash = Ethereum_Module.callContractMethod({
    address: resolverAddress,
    method: "function setContenthash(bytes32 node, bytes hash)",
    args: [namehash(input.domain), input.cid],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return setContentHash.unwrap();
}

export function deployFIFSRegistrar(input: Input_deployFIFSRegistrar): string {
  const address = Ethereum_Module.deployContract({
    abi,
    bytecode,
    args: [input.registryAddress, namehash(input.tld)],
    connection: input.connection,
  });

  return address.unwrap();
}

export function registerSubnodeOwnerWithFIFSRegistrar(
  input: Input_registerSubnodeOwnerWithFIFSRegistrar
): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const txHash = Ethereum_Module.callContractMethod({
    address: input.fifsRegistrarAddress,
    method: "function register(bytes32 label, address owner) external",
    args: [keccak256(input.label), input.owner],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return txHash.unwrap();
}

export function setTextRecord(input: Input_setTextRecord): Ethereum_TxResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;
  const txHash = Ethereum_Module.callContractMethod({
    address: input.resolverAddress,
    method: "function setText(bytes32 node, string key, string value)",
    args: [namehash(input.domain), input.key, input.value],
    connection: input.connection,
    txOverrides: {
      gasLimit: txOverrides.gasLimit,
      gasPrice: txOverrides.gasPrice,
      value: null,
    },
  });

  return txHash.unwrap();
}

export function configureOpenDomain(
  input: Input_configureOpenDomain
): ConfigureOpenDomainResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;

  const fifsRegistrarAddress = deployFIFSRegistrar({
    registryAddress: input.registryAddress,
    tld: input.tld,
    connection: input.connection,
    txOverrides,
  });

  const splitDomain = input.tld.split(".");
  const tldLabel = splitDomain[0];
  const tld = splitDomain.slice(1, splitDomain.length).join(".");

  const registerOpenDomainTxReceipt = registerDomain({
    registrarAddress: input.registrarAddress,
    registryAddress: input.registryAddress,
    domain: input.tld,
    owner: input.owner,
    connection: input.connection,
    txOverrides,
  });

  const setSubdomainRecordTxReceipt = setSubdomainRecord({
    domain: tld,
    label: tldLabel,
    owner: fifsRegistrarAddress,
    registryAddress: input.registryAddress,
    resolverAddress: input.resolverAddress,
    ttl: "0",
    connection: input.connection,
    txOverrides,
  });

  return {
    fifsRegistrarAddress,
    registerOpenDomainTxReceipt,
    setSubdomainRecordTxReceipt,
  };
}

export function createSubdomainInOpenDomain(
  input: Input_createSubdomainInOpenDomain
): CreateSubdomainInOpenDomainResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;

  const registerSubdomainTxReceipt = registerSubnodeOwnerWithFIFSRegistrar({
    label: input.label,
    owner: input.owner,
    fifsRegistrarAddress: input.fifsRegistrarAddress,
    connection: input.connection,
    txOverrides,
  });

  const setResolverTxReceipt = setResolver({
    domain: input.label + "." + input.domain,
    registryAddress: input.registryAddress,
    resolverAddress: input.resolverAddress,
    connection: input.connection,
    txOverrides,
  });

  return { registerSubdomainTxReceipt, setResolverTxReceipt };
}

export function createSubdomainInOpenDomainAndSetContentHash(
  input: Input_createSubdomainInOpenDomainAndSetContentHash
): CreateSubdomainInOpenDomainAndSetContentHashResponse {
  const txOverrides: TxOverrides =
    input.txOverrides === null
      ? { gasLimit: null, gasPrice: null }
      : input.txOverrides!;

  const createSubdomainInOpenDomainTxReceipt = createSubdomainInOpenDomain({
    label: input.label,
    domain: input.domain,
    resolverAddress: input.resolverAddress,
    registryAddress: input.registryAddress,
    owner: input.owner,
    fifsRegistrarAddress: input.fifsRegistrarAddress,
    connection: input.connection,
    txOverrides,
  });

  const setContentHashReceiptTx = setContentHash({
    domain: input.label + "." + input.domain,
    cid: input.cid,
    resolverAddress: input.resolverAddress,
    connection: input.connection,
    txOverrides,
  });

  return {
    registerSubdomainTxReceipt: createSubdomainInOpenDomainTxReceipt.registerSubdomainTxReceipt,
    setResolverTxReceipt: createSubdomainInOpenDomainTxReceipt.setResolverTxReceipt,
    setContentHashReceiptTx,
  };
}
