<template>
  <div class=" mx-auto">
  <ul class="divide-y divide-gray-200">
    <li v-for="spider in spiders" :key="spider.token" class="py-4 flex justify-between items-center">
      <div class="flex-1">
        <div class="font-bold  text-gray-900">Username: {{ spider.username }}</div>
        <div class=" text-gray-500">Date: {{ spider.date }}</div>
        <div class=" text-gray-500">Country Code: {{ spider.country_code }}</div>
        <div class=" text-red-500">Failed Number: {{ spider.page_number * 100 }}</div>
      </div>
      <button @click="rerunSpider(spider)" class="inline-flex items-center px-4 py-2 border border-transparent font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">
        Rerun
      </button>
    </li>
  </ul>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

type Spider = {
    username: string;
    password: string;
    date: string;
    country_code: string;
    token: string;
    page_number: number;
};

const spiders = ref<Spider[]>([
  // Example spider data
  {
    username: 'user1',
    password: 'pass1',
    date: '2023-01-01',
    country_code: 'US',
    token: 'token1',
    page_number: 1,
  },
  // Add more spider objects here
]);

const rerunSpider = (spider: Spider) => {
  // Implement the logic to rerun the spider based on its token
  
};
</script>