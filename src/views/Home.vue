<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { Spider } from "../mod";
import { useRoute } from "vue-router";


const route = useRoute();

// Create a new Date object for the current date
const today = new Date();

onMounted(async () => {
  if (route.query.spider_str) {
    let spider: Spider = JSON.parse(route.query.spider_str as string);
    console.log("spider_str: ", spider);
    username.value = spider.username;
    password.value = spider.password;
    date.value = spider.date;
    country.value = spider.country_code;
    pageNumber.value = spider.page_number;
  }
});

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
const pageNumber = ref(1);

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

  // start one task, 0.5s invoke the function spider_status
  const interval = setInterval(async () => {
    const response = await invoke("spider_status");
    formResponse.value = response as string;
  }, 500);
  // note: the pageNumber argument is equation the rust page_number argument.
  const response = await invoke("spider_start", {
    username: username.value,
    password: password.value,
    date: date.value,
    country: country.value,
    pageNumber: pageNumber.value,
  }).catch(() => {
    clearInterval(interval);
  });

  clearInterval(interval);

  if (response === undefined) {
    formResponse.value = "the network is break, go Fail page to rerun the task";
  } else {
    formResponse.value += ", The data store folder: Desktop/data, Spider finished: " + response;
  }

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
          <option value="">All</option>
          <option v-for="country in countries" :key="country.value" :value="country.value">
            {{ country.text }}
          </option>
          <!-- Add more countries as needed -->
        </select>
      </div>
      <div>
        <button type="submit" :disabled="isProcessing"
          class="w-full flex justify-center py-3 px-6 border border-transparent rounded-lg shadow-sm text-lg font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
          {{ isProcessing ? 'Processing...' : 'Spider' }}
        </button>
      </div>
    </form>
    <div class="mt-4 p-4 bg-slate-500 rounded">
      {{ formResponse }}
    </div>
  </div>
</template>
