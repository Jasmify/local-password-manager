<script lang="ts">
  import { SearchIcon } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import { searchCriteriaStore } from "../store";
  import type { SearchCriteria } from "../models";

  let searchCriteria: SearchCriteria = {
    accountName: "",
    identifier: "",
    categoryName: "",
  };

  function handleSubmit(event: Event) {
    event.preventDefault();
    // ストアにsearchCriteriaを保存
    searchCriteriaStore.set(searchCriteria);

    goto("/result");
  }
</script>

<div
  class="max-w-md mx-auto p-6 bg-white rounded-lg shadow-md dark:bg-gray-800"
>
  <form on:submit={handleSubmit} class="space-y-4">
    <div class="space-y-2">
      <label
        for="accountName"
        class="text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        Account Name
      </label>
      <input
        type="text"
        id="accountName"
        bind:value={searchCriteria.accountName}
        placeholder="Enter account name"
        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
      />
    </div>
    <div class="space-y-2">
      <label
        for="id"
        class="text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        ID
      </label>
      <input
        type="text"
        id="id"
        bind:value={searchCriteria.identifier}
        placeholder="Enter ID"
        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
      />
    </div>
    <div class="space-y-2">
      <label
        for="category"
        class="text-sm font-medium text-gray-700 dark:text-gray-200"
      >
        Category
      </label>
      <input
        type="text"
        id="category"
        bind:value={searchCriteria.categoryName}
        placeholder="Enter category"
        class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
      />
    </div>
    <button
      type="submit"
      class="w-full flex items-center justify-center px-4 py-2 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-teal-600 hover:bg-teal-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
    >
      <SearchIcon class="w-5 h-5 mr-2" />
      Search
    </button>
  </form>
</div>
