<script lang="ts">
  import { z } from "zod";
  import { PlusCircle, XCircle } from "lucide-svelte";
  import { formSchema } from "../models";
  import type { FormData } from "../models";

  export let isEdit;
  export let form: FormData;
  export let onSubmit: (event: { detail: FormData }) => void;

  let errors: Partial<Record<keyof FormData, string>> = {};

  function validateField(field: keyof FormData) {
    const fieldSchema = formSchema.shape[field];
    try {
      fieldSchema.parse(form[field]);
      errors[field] = undefined;
    } catch (error) {
      if (error instanceof z.ZodError) {
        errors[field] = error.errors[0].message;
      }
    }
  }

  function validateForm() {
    try {
      formSchema.parse(form);
      errors = {};
      return true;
    } catch (error) {
      if (error instanceof z.ZodError) {
        error.errors.forEach((err) => {
          errors[err.path[0] as keyof FormData] = err.message;
        });
      }
      return false;
    }
  }

  function handleSubmit(event: Event) {
    event.preventDefault();
    if (validateForm()) {
      onSubmit?.({ detail: form });
    }
  }

  function addPassword() {
    form.passwords = [...form.passwords, ""];
  }

  function removePassword(index: number) {
    form.passwords = form.passwords.filter((_, i) => i !== index);
    validateField("passwords");
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-4">
  <div>
    <label
      for="accountName"
      class="block text-sm font-medium text-gray-700 mb-1">Account Name</label
    >
    <input
      id="account_name"
      name="account_name"
      type="text"
      bind:value={form.accountName}
      on:blur={() => validateField("accountName")}
      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
    />
    {#if errors.accountName}
      <p class="text-red-500 text-sm mt-1">{errors.accountName}</p>
    {/if}
  </div>
  <div>
    <label for="identifier" class="block text-sm font-medium text-gray-700 mb-1"
      >ID</label
    >
    <input
      id="identifier"
      name="identifier"
      type="text"
      bind:value={form.identifier}
      on:blur={() => validateField("identifier")}
      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
    />
    {#if errors.identifier}
      <p class="text-red-500 text-sm mt-1">{errors.identifier}</p>
    {/if}
  </div>
  <div>
    <div class="block text-sm font-medium text-gray-700 mb-1">Password</div>
    {#each form.passwords as _, index}
      <div class="flex items-center space-x-2 mt-2">
        <input
          type="text"
          bind:value={form.passwords[index]}
          on:blur={() => validateField("passwords")}
          placeholder={`Password ${index + 1}`}
          aria-label={`Password ${index + 1}`}
          class="flex-grow px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
        />
        {#if index === form.passwords.length - 1 && form.passwords.length > 1}
          <button
            type="button"
            on:click={() => removePassword(index)}
            class="p-2 text-gray-400 hover:text-gray-500 focus:outline-none"
          >
            <XCircle class="h-5 w-5" />
          </button>
        {/if}
      </div>
    {/each}
    {#if errors.passwords}
      <p class="text-red-500 text-sm mt-1">{errors.passwords}</p>
    {/if}
    <button
      type="button"
      on:click={addPassword}
      class="mt-2 inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
    >
      <PlusCircle class="h-4 w-4 mr-2" />
      Add Password
    </button>
  </div>
  <div>
    <label for="category" class="block text-sm font-medium text-gray-700 mb-1"
      >Category</label
    >
    <input
      id="category"
      name="category"
      type="text"
      bind:value={form.categoryName}
      on:blur={() => validateField("categoryName")}
      class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
    />
    {#if errors.categoryName}
      <p class="text-red-500 text-sm mt-1">{errors.categoryName}</p>
    {/if}
  </div>
  <button
    type="submit"
    class={`w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium focus:outline-none focus:ring-2 focus:ring-offset-2 
      ${
        isEdit
          ? "bg-rose-600 hover:bg-rose-700 text-white"
          : "bg-indigo-600 hover:bg-indigo-700 text-white"
      }`}
  >
    {isEdit ? "Edit" : "Register"}
  </button>
</form>
