import { TezosPlugin as Plugin } from ".";
import { Query, Mutation } from "./w3";
import * as Types from "./w3";

export const mutation = (plugin: Plugin): Mutation.Module => ({
  callContractMethod: async (
    input: Mutation.Input_callContractMethod
  ): Promise<Types.TxOperation> => {
    return plugin.callContractMethod(input);
  },

  callContractMethodAndConfirmation: async (
    input: Mutation.Input_callContractMethodAndConfirmation
  ): Promise<Types.CallContractMethodConfirmationResponse> => {
    return plugin.callContractMethodAndConfirmation(input);
  },

  transfer: async (input: Mutation.Input_transfer): Promise<string> => {
    return plugin.transfer(input);
  },

  transferAndConfirm: async (
    input: Mutation.Input_transferAndConfirm
  ): Promise<Types.TransferConfirmation> => {
    return plugin.transferAndConfirm(input);
  },

  signMessage: async (
    input: Mutation.Input_signMessage
  ): Promise<Types.SignResult> => {
    return plugin.signMessage(input);
  },

  originate: async (
    input: Mutation.Input_originate
  ): Promise<Types.OriginationResponse> => {
    return plugin.originate(input);
  },

  originateAndConfirm: async (
    input: Mutation.Input_originateAndConfirm
  ): Promise<Types.OriginationConfirmationResponse> => {
    return plugin.originateAndConfirm(input);
  },
});

export const query = (plugin: Plugin): Query.Module => ({
  getContractStorage: async (
    input: Query.Input_getContractStorage
  ): Promise<string> => {
    return plugin.getContractStorage(input);
  },

  getPublicKey: async (
    input: Query.Input_getPublicKey
  ): Promise<string> => {
    return plugin.getPublicKey(input);
  },

  getPublicKeyHash: async (
    input: Query.Input_getPublicKeyHash
  ): Promise<string> => {
    return plugin.getPublicKeyHash(input);
  },

  getRevealEstimate: async (
    input: Query.Input_getRevealEstimate
  ): Promise<Types.EstimateResult> => {
    return plugin.getRevealEstimate(input);
  },

  getTransferEstimate: async (
    input: Query.Input_getTransferEstimate
  ): Promise<Types.EstimateResult> => {
    return plugin.getTransferEstimate(input);
  },

  getOriginateEstimate: async (
    input: Query.Input_getOriginateEstimate
  ): Promise<Types.EstimateResult> => {
    return plugin.getOriginateEstimate(input);
  },

  checkAddress: async (input: Query.Input_checkAddress): Promise<boolean> => {
    return plugin.checkAddress(input);
  },

  getBalance: async (input: Query.Input_getBalance): Promise<string> => {
    return plugin.getBalance(input);
  }
});