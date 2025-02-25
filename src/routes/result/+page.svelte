<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Table from "$lib/Table.svelte";
  import { searchCriteriaStore } from "../../store";
  import type { SearchCriteria, AccountSummary } from "../../models";

  let searchResults: Array<AccountSummary> = [];

  async function fetchSearchResults(searchCriteria: SearchCriteria) {
    searchResults = await invoke("get_search_results", { searchCriteria });
  }

  // searchCriteriaStore の変更を感知して関数を実行
  $: {
    const searchCriteria = $searchCriteriaStore;
    if (searchCriteria) {
      fetchSearchResults(searchCriteria);
    }
  }
</script>

<Table data={searchResults} />
