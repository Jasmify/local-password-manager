<script lang="ts">
  import Form from "$lib/Form.svelte";
  import type { FormData } from "../../models";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  let form: FormData = {
    accountName: "",
    identifier: "",
    passwords: [""],
    categoryName: "Other",
  };

  async function handleFormSubmit(event: { detail: FormData }) {
    const formData = event.detail;
    await invoke<void>("insert_form_data", { formData: formData });
    goto("/");
  }
</script>

<div
  class="w-full max-w-md mx-auto bg-white shadow-md rounded-lg overflow-hidden"
>
  <div class="p-6">
    <h2 class="text-2xl font-bold mb-2">Account Registration</h2>
    <Form isEdit={false} {form} onSubmit={handleFormSubmit} />
  </div>
</div>
