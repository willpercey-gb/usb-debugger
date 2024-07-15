<template>
  <div class="root">
    <button @click.prevent="reload()">Refresh</button>
    <h2>Device List: </h2>
    <div class="bulma">
      <div class="card" style="box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" v-for="device in devices">
        <div class="card-content">
          <p>Device: {{ device.vendor }} {{ device.name }}</p>
          <p>Path:  {{ device.path }}</p>
          <p>Serial No: {{ device.serial_number }}</p>
        </div>
      </div>
    </div>
    <hr/>
    <h2>Process List:</h2>
<!--    TODO -->
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';


export default defineComponent({
  name: 'App',
  mounted() {
    this.listDevices();
  },
  data: () => ({
    //objects to allow for extension later
    devices: null as [{ name: string, path: string, vendor: string, serial_number: string }] | null,
  }),
  methods: {
    reload() {
      window.location.reload()
    },
    async listDevices() {
      try {
        const usbInfo = await invoke('list_usb_devices');
        console.log(usbInfo);
        this.devices = usbInfo ? JSON.parse(usbInfo) : null;
      } catch (error) {
        console.error(error);
      }
    }
  }
});
</script>