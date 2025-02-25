<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { accountInfoStore } from "../../store";
  import Form from "$lib/Form.svelte";
  import type { FormData, AccountInfo } from "../../models";
  import { Trash2Icon } from "lucide-svelte";

  let form: FormData = {
    accountName: "",
    identifier: "",
    passwords: [""],
    categoryName: "",
  };

  let accountInfo: AccountInfo | null = null;

  accountInfoStore.subscribe((oldAccountInfo: AccountInfo | null) => {
    if (oldAccountInfo) {
      accountInfo = oldAccountInfo;
      const { accountName, identifier, passwords, categoryName } =
        oldAccountInfo;
      form = {
        accountName,
        identifier,
        passwords: passwords.map((passwordInfo) => passwordInfo.passwordRaw),
        categoryName,
      };
    }
  });

  async function handleFormSubmit(event: { detail: FormData }) {
    const formData = event.detail;
    if (accountInfo) {
      await invoke<void>("update_account_info", { formData, accountInfo });
    }

    goto("/");
  }

  async function handleDelete() {
    let confirm = window.confirm(
      "Are you sure you want to delete this item?\nThis action cannot be undone."
    );
    if (confirm) {
      let accountUlid = accountInfo?.accountUlid;
      await invoke<void>("delete_account", { accountUlid });
      goto("/");
    }
  }
</script>

<div
  class="w-full max-w-md mx-auto bg-white shadow-md rounded-lg overflow-hidden"
>
  <div class="w-full flex justify-end p-2">
    <button
      type="button"
      class="text-gray-400 hover:text-red-700"
      on:click={handleDelete}
    >
      <Trash2Icon />
    </button>
  </div>
  <div class="p-6">
    <h2 class="text-2xl font-bold mb-2">Details</h2>
    <Form isEdit={true} {form} onSubmit={handleFormSubmit} />
  </div>
</div>
