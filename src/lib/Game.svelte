<script lang="ts">
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  type Card = {
    color: string;
    isKey: boolean;
    isFlipped: boolean;
    imagePath: string;
  };

  const cardColors = [
    "bg-red-100",
    "bg-blue-100",
    "bg-green-100",
    "bg-yellow-100",
  ];
  const cardImages = [
    "./cat01.png",
    "./cat02.png",
    "./cat03.png",
    "./cat04.png",
  ];

  let cards: Card[] = [];
  let keyFound = false;

  function initializeCards(): Card[] {
    const shuffledColors = [...cardColors].sort(() => Math.random() - 0.5);
    const shuffledImages = [...cardImages].sort(() => Math.random() - 0.5);
    const randomKeyIndex = Math.floor(Math.random() * 4);

    return shuffledColors.map((color, index) => ({
      color,
      isKey: index === randomKeyIndex,
      isFlipped: false,
      imagePath: shuffledImages[index],
    }));
  }

  function resetGame() {
    cards = initializeCards();
    keyFound = false;
  }

  function handleClick(index: number) {
    if (keyFound) return;

    cards = cards.map((card, i) => {
      if (i === index) {
        if (card.isKey) {
          keyFound = true;
        }
        return { ...card, isFlipped: true };
      }
      return card;
    });
  }

  $: if (keyFound) {
    setTimeout(() => {
      resetGame();
    }, 2000);
  }

  onMount(() => {
    resetGame();
  });
</script>

<div class="flex justify-center">
  <div class="grid grid-cols-2 gap-4 p-4">
    {#each cards as card, index}
      <div
        role="button"
        tabindex="0"
        class="w-[25vh] h-[25vh] m-2 {card.color} shadow-md rounded-lg overflow-hidden cursor-pointer flex items-center justify-center text-4xl font-bold transition-all duration-300 {card.isFlipped
          ? 'border-2 border-purple-500'
          : ''}"
        on:click={() => handleClick(index)}
        on:keydown={(e) => e.key === "Enter" && handleClick(index)}
      >
        {#if !card.isFlipped}
          <img
            src={card.imagePath}
            alt="Card {index + 1}"
            class="w-full h-full object-cover"
            transition:fade
          />
        {:else if card.isKey}
          <img
            src="./cat_key.png"
            alt="Key"
            class="w-full h-full object-cover"
          />
        {:else}
          <img
            src="./cat_empty.png"
            alt="Card Back"
            class="w-full h-full object-cover"
          />
        {/if}
      </div>
    {/each}
  </div>
</div>
