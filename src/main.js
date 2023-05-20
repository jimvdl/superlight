const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

window.addEventListener("DOMContentLoaded", () => {
  listen('battery-update', (event) => {
    console.log(event.payload);
    document.querySelector("#percentage").textContent = event.payload.percentage;
    document.querySelector("#charging").textContent = event.payload.charging;
  })
});
