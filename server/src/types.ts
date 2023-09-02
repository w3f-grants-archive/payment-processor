import { Router } from "express";

/**
 * Represents the routes
 */
export interface Routes {
  path?: string;
  router: Router;
}

// Request body for the POS
export interface RequestBody {
  amount: string;
  cardNumber: string;
  cardExpiration: string;
  cvv: string;
}

// Common MTI types
export enum MTI {
  // Authorization Request from the POS, for any online transactions
  AuthorizationRequest = "0100",
  // Authorization Response from the server
  AuthorizationRequestResponse = "0110",
  // Financial Transaction Request from the POS, usually for ATMs
  FinancialTransactionRequest = "0200",
  // Financial Transaction Response from the server
  FinancialTransactionRequestResponse = "0210",
  // Reversal request
  ReversalRequest = "0400",
  // Reversal response
  ReversalRequestResponse = "0410",
}

// Merchant Category Codes
// Currently, we only support the following MCCs
export enum MCC {
  // Utility services
  ComputerNetworkServices = "4816",
  // Wire transfer
  WireTransfer = "4829",
  // Commercial art, graphics and photography
  CommercialArtGraphicsPhotography = "7333",
}

// Hard coded processing codes
export enum ProcessingCode {
  // Purchase from any account
  Purchase = "000000",
  // Purchase from savings account
  PurchaseFromSavingsAccount = "001000",
  // Purchase from checking account
  PurchaseFromCheckingAccount2 = "002000",
  // Withdrawal from any account
  Withdrawal = "010000",
}
