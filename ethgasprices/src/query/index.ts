import { JSON, Result, BigInt } from "@web3api/wasm-as";

import {
  Http_Header,
  Http_Query,  
  Http_Request,
  Http_ResponseType,
  Http_UrlParam,
  HistoricalEthereumGasPrice,
  Input_getGasData
} from "./w3";


export function getGasData(input: Input_getGasData) : HistoricalEthereumGasPrice {
  const date = input.date; 

  return {
    date : date,
    gasPriceInWei : getGasPrice(date).unwrap()
  };
}

function getGasPrice(date : string) : Result<BigInt, string> {
  const csvContent = fetchGasPricesContentFromEtherScan().unwrap();

  const gasPricesMap = parseGasPricesContent(csvContent).unwrap();
  if (gasPricesMap.has(date))
  {
    const gasPrice = gasPricesMap.get(date);
    return Result.Ok<BigInt, string>(gasPrice);
  }
  return Result.Err<BigInt, string>(`No gas price for given date ${date}`);
}

function parseGasPricesContent(csvContent : string) : Result<Map<string, BigInt>, string> {

  if (!csvContent || csvContent.length == 0)
  {
    return Result.Err<Map<string, BigInt>, string>(`ERROR_CSV_CONTENT: Got empty csv content`);
  }

  // Parse CSV into map. 
  const gasPriceMap = new Map<string, BigInt>();

  const lines = csvContent.split("\n");
  const linesCount = csvContent.length;

  // Ignore first line(header).
  for (var i = 1; i<lines.length; i++)
  {
    const line = lines[i];
    
    // Ignore empty lines.
    if (line.trim().length == 0)
    {
      continue;
    }

    const fields = line.split(",");
    if (fields.length != 3)
    {
      return Result.Err<Map<string, BigInt>, string>(`ERROR_FORMAT: Expecting three fields. Got ${line} lineNumber=${i+1}`);
    }

    let parsedDate = fields[0].trim();
    parsedDate = parsedDate.substring(1, parsedDate.length - 1);

    let unixTimestamp = fields[1].trim();
    unixTimestamp = unixTimestamp.substring(1, unixTimestamp.length - 1);

    let gasPriceWei = fields[2].trim();
    gasPriceWei = gasPriceWei.substring(1, gasPriceWei.length - 1);
    gasPriceMap.set(parsedDate, BigInt.fromString(gasPriceWei));
  }

  return Result.Ok<Map<string, BigInt>, string>(gasPriceMap);
}

function fetchGasPricesContentFromEtherScan() : Result<string, string> {
  const response = Http_Query.get({
    url: " https://etherscan.io/chart/gasprice",
    request: {
      responseType: Http_ResponseType.TEXT,
      headers: [{
        key: "user-agent",
        value: "EthGasPriceWrapper"
      }],
      urlParams: [{
        key: "output",
        value: "csv"
      }],
      body: ""
    }
  }).unwrap();
  
  if (!response) {
    return Result.Err<string, string>("ERROR_NO_RESPONSE: fetchGasPricesFromEtherScan");
  }

  return Result.Ok<string, string>(response.body!);
}

