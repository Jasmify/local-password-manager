import { writable } from "svelte/store";
import type { SearchCriteria, AccountInfo } from "./models";

export const searchCriteriaStore = writable<SearchCriteria>({
  accountName: "",
  identifier: "",
  categoryName: "",
});

export const accountInfoStore = writable<AccountInfo | null>(null);
