<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { accountInfoStore } from "../store";
  import { EyeIcon, EyeOffIcon } from "lucide-svelte";
  import type { AccountInfo, AccountSummary, PasswordInfo } from "../models";

  export let data: AccountSummary[];

  let revealedPasswords: { [key: string]: PasswordInfo[] } = {};

  // 表示ボタンを押したら、identifierUlidを元に、復号化して生のパスワードを返す処理
  async function handleRevealPassword(identifierUlid: string) {
    try {
      const passwords = await invoke<PasswordInfo[]>("get_password_info", {
        identifierUlid,
      });
      revealedPasswords[identifierUlid] = passwords;
      revealedPasswords = revealedPasswords; // Trigger reactivity
    } catch (error) {
      // ユーザーに分かりやすいエラーメッセージを表示
      alert("An error occurred while retrieving the password.");
    }
  }

  function hidePassword(identifierUlid: string) {
    delete revealedPasswords[identifierUlid];
    revealedPasswords = revealedPasswords; // Trigger reactivity
  }

  async function navigateToDetail(accountSummary: AccountSummary) {
    let identifierUlid = accountSummary.identifierUlid;
    if (!revealedPasswords[identifierUlid]) {
      revealedPasswords[identifierUlid] = await invoke<PasswordInfo[]>(
        "get_password_info",
        { identifierUlid }
      );
    }
    const passwords = revealedPasswords[identifierUlid];

    let accountInfo: AccountInfo = {
      ...accountSummary,
      passwords,
    };
    accountInfoStore.set(accountInfo);

    goto("/detail");
  }
</script>

<div class="overflow-x-auto">
  <table class="w-full mb-8 divide-y divide-gray-300">
    <thead class="bg-gray-300">
      <tr>
        <th class="text-left text-sm p-2">No.</th>
        <th class="text-left text-sm p-2">Account Name</th>
        <th class="text-left text-sm p-2">ID</th>
        <th class="text-left text-sm p-2">Password</th>
        <th class="text-left text-sm p-2">Category</th>
        <th class="text-left text-sm p-2">Details</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-gray-300">
      {#each data as item, index (item.accountUlid)}
        <tr class="hover:bg-gray-200">
          <td class="p-2">{index + 1}</td>
          <td class="p-2">{item.accountName}</td>
          <td class="p-2">{item.identifier}</td>
          <td class="p-2">
            <div class="flex justify-between items-start">
              <div class="space-y-2 flex-grow mr-4">
                {#if revealedPasswords[item.identifierUlid]}
                  {#each revealedPasswords[item.identifierUlid] as password}
                    <div class="break-all">{password.passwordRaw}</div>
                  {/each}
                {:else}
                  <div class="text-gray-500">*****</div>
                {/if}
              </div>
              <div class="flex-shrink-0">
                {#if revealedPasswords[item.identifierUlid]}
                  <button
                    class="inline-flex items-center px-2 py-1 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    on:click={() => hidePassword(item.identifierUlid)}
                  >
                    <EyeOffIcon class="h-4 w-4 mr-2" />
                    Hide
                  </button>
                {:else}
                  <button
                    class="inline-flex items-center px-2 py-1 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                    on:click={() => handleRevealPassword(item.identifierUlid)}
                  >
                    <EyeIcon class="h-4 w-4 mr-2" />
                    Show
                  </button>
                {/if}
              </div>
            </div>
          </td>
          <td class="p-2">{item.categoryName}</td>
          <td class="p-2">
            <button
              class="inline-flex items-center px-2 py-1 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
              on:click={() => navigateToDetail(item)}
            >
              Details
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
