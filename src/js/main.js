const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;
const { appWindow } = window.__TAURI__.window;

$(document).ready(() => {
  $("#titlebar-close").click(() => {
    appWindow.hide()
  });

  $("#battery").hide();
  // $("#loader").hide();
  
  listen('battery-update', (event) => {
    console.log(event.payload);

    var battery = event.payload.percentage;
    var src;
    if (battery >= 80) {
      src = "battery-full";
    }

    if (battery >= 25 && battery < 80) {
      src = "battery-half";
    }

    if (battery < 25) {
      src = "battery-empty";
    }

    if (event.payload.charging) {
      src = "battery-charging";
    }

    $("#percentage").text(event.payload.percentage);
    $("#battery-icon").attr("src", "assets/" + src + ".svg");
    $("#loader").fadeOut();
    $("#loader").promise().done(() => {
      $("#battery").fadeIn();
    })
  })
});

// window.addEventListener("DOMContentLoaded", () => {
//   $("titlebar-close").on("click", () => {
//     appWindow.hide()
//   });
//   // document
//   //   .getElementById('titlebar-close')
//   //   .addEventListener('click', () => appWindow.hide())

//   batteryEl = document.getElementById("battery");
//   //batteryEl.style.display = "none";
//   batteryIconEl = document.getElementById("battery-icon");
//   // batteryIconEl.style.display = "none";

//   listen('battery-update', (event) => {
//     console.log(event.payload);

//     document.querySelector("#percentage").textContent = event.payload.percentage;

//     document.getElementById("loader").style.display = "none";
//     batteryEl.style.display = "block";
//     batteryIconEl.style.display = "block";
//   })
// });
