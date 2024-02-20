import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { ViteRsw } from 'vite-plugin-rsw'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue(), ViteRsw()],
    build: {
        rollupOptions: {
            output: {
                manualChunks(id) {
                    if (id.indexOf("node_modules") != -1) {
                        return id.toString().split("node_modules/")[1].split("/")[0].toString();
                    }
                },
            },
        },
    },
});
