<script setup lang="ts">
import SumComponent from "./components/SumComponent.vue";
</script>

<template>

  <el-row class="row-bg" justify="center">
    <el-col :span="2"></el-col>
    <el-col :span="20"><SumComponent></SumComponent></el-col>
    <el-col :span="2"></el-col>
  </el-row>
  
</template>
