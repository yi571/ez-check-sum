<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { DocumentCopy } from '@element-plus/icons-vue'
import { open } from '@tauri-apps/api/dialog';
import { ElMessage } from 'element-plus'

const check_md5 = ref(true)
const check_sha256 = ref(true)
const check_sha512 = ref(true)

const data_md5 = ref('')
const data_sha256 = ref('')
const data_sha512 = ref('')

const running = ref('選擇檔案')

async function select_file() {
    console.log("check_md5", check_md5)
    console.log("check_sha256", check_sha256)
    console.log("check_sha256", check_sha256)
    if (!check_md5.value && !check_sha256.value && !check_sha512.value) {
        ElMessage({
            message: '請至少選擇一項！',
            type: 'warning',
        })
        return
    }

    const selected = await open({
        multiple: false,
    })

    if (selected === null) {
        return
    }

    data_md5.value = ''
    data_sha256.value = ''
    data_sha512.value = ''
    running.value = '計算中...'
    var data = await invoke<HashData>(
        "get_hash",
        { path: selected, checkmd5: check_md5.value, checksha256: check_sha256.value, checksha512: check_sha512.value }
    )
    data_md5.value = data.md5
    data_sha256.value = data.sha256
    data_sha512.value = data.sha512
    running.value = selected.toString()
}

async function copy_to_clipboard(data: string) {
    navigator.clipboard.writeText(data)
        .then(() => {
            ElMessage({
                message: '已複製到剪貼簿',
                type: 'success',
            })
        })
}

class HashData {
    md5!: string;
    sha256!: string;
    sha512!: string;
}

</script>

<template>
    <el-checkbox v-model="check_md5" :disabled="running == '計算中...'" >MD5</el-checkbox>
    <el-checkbox v-model="check_sha256" :disabled="running == '計算中...'" >SHA256</el-checkbox>
    <el-checkbox v-model="check_sha512" :disabled="running == '計算中...'" >SHA512</el-checkbox>
    <el-space fill wrap direction="vertical" style="width: 100%">


        <el-input v-show="check_md5" v-model="data_md5" disabled :key="1">
            <template #prepend>MD5</template>
            <template #append>
                <el-button :icon="DocumentCopy" @click="copy_to_clipboard(data_md5)" :disabled="data_md5 == ''" />
            </template>
        </el-input>

        <el-input v-show="check_sha256" v-model="data_sha256" disabled :key="2">
            <template #prepend>SHA256</template>
            <template #append>
                <el-button :icon="DocumentCopy" @click="copy_to_clipboard(data_sha256)" :disabled="data_sha256 == ''" />
            </template>
        </el-input>

        <el-input v-show="check_sha512" v-model="data_sha512" disabled :key="3">
            <template #prepend>SHA512</template>
            <template #append>
                <el-button :icon="DocumentCopy" @click="copy_to_clipboard(data_sha512)" :disabled="data_sha512 == ''" />
            </template>
        </el-input>

        <el-button @click="select_file()" :disabled="running == '計算中...'" :key="4">{{ running }}</el-button>
    </el-space>

</template>