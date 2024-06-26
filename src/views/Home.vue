<script setup lang="ts">
import { ref} from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// Create a new Date object for the current date
const today = new Date();

// Subtract one day from the current date
today.setDate(today.getDate() - 1);

// Format the date as YYYY-MM-DD
const formattedDate = today.toISOString().split('T')[0];


const username = ref("");
const password = ref("");
const date = ref(formattedDate);
const country = ref("All");
const formResponse = ref("")
const isProcessing = ref(false);

const countries = ref([
  { value: 'Brazil', text: 'Brazil' },
  { value: 'India', text: 'India' },
  { value: 'Indonesia', text: 'Indonesia' },
  { value: 'Philippines', text: 'Philippines' },
  { value: 'Pakistan', text: 'Pakistan' },
  // Add more options as needed
]);

const handleSubmit = async () => {
  formResponse.value = "Starting spider..., wait for the time.\n"
  isProcessing.value = true;
  console.log("Starting spider...");
  

  const response = await invoke("spider_start", {
    username: username.value,
    password: password.value,
    date: date.value,
    country: country.value,
  });

  formResponse.value = "The data store folder: Desktop/data, Spider finished: " + response;
  isProcessing.value = false;
};

</script>

<template>
  <div class="mx-auto mt-10">
    <form @submit.prevent="handleSubmit" class="p-8 bg-white shadow-lg rounded-lg">
      <div class="mb-6">
        <label for="username" class="block text-lg font-semibold text-gray-700">Username</label>
        <input v-model="username" type="text" id="username"
          class="mt-2 block w-full p-3 border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
      </div>
      <div class="mb-6">
        <label for="password" class="block text-lg font-semibold text-gray-700">Password</label>
        <input v-model="password" type="password" id="password"
          class="mt-2 block w-full p-3 border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
      </div>
      <div class="mb-6">
        <label for="date" class="block text-lg font-semibold text-gray-700">Date</label>
        <input v-model="date" type="date" id="date"
          class="mt-2 block w-full p-3 border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
      </div>
      <div class="mb-8">
        <label for="country" class="block text-lg font-semibold text-gray-700">Country</label>
        <select v-model="country" id="country"
          class="mt-2 block w-full p-3 border-gray-300 rounded-lg shadow-sm focus:border-indigo-500 focus:ring-indigo-500">
          <option value="All">All</option>
          <option v-for="country in countries" :key="country.value" :value="country.value">
            {{ country.text }}
          </option>
          <!-- Add more countries as needed -->
        </select>
      </div>
      <div>
        <button type="submit"
         :disabled="isProcessing"
          class="w-full flex justify-center py-3 px-6 border border-transparent rounded-lg shadow-sm text-lg font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          {{ isProcessing ? 'Processing...' : 'Spider' }}
        </button>
      </div>
    </form>
    <div  class="mt-4 p-4 bg-slate-500 rounded">
      {{ formResponse }}
    </div>
  </div>
</template>
