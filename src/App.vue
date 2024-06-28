<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

</script>

<template>
  <div class="mx-auto px-4">
    <nav class="bg-gray-800 text-white p-4 rounded-md">
      <ul class="flex space-x-4">
        <li><router-link to="/" class="hover:text-gray-300">Home</router-link></li>
        <li><router-link to="/fail" class="hover:text-gray-300">Fail</router-link></li>
        <li><router-link to="/about" class="hover:text-gray-300">About</router-link></li>
      </ul>
    </nav>
    <router-view />
  </div>
</template>