import { listen } from '@tauri-apps/api/event'

// Listen Install Progress
// https://tauri.app/v1/guides/distribution/updater#listen-install-progress
listen('tauri://update-status', function (res) {
  console.log('tauri://update-status', res)
})
