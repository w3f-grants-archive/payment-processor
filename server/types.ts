export interface PaginationOptions {
  pageNumber: number;
  pageSize: number;
  maxPageSize?: number;
}

// EraStat type to return
export interface EraStat {
  eraIndex: number;
  startBlock: number;
  endBlock: number;
  totalRewards: string;
  totalSlashes: string;
}
