/**
 * compression.js — public joke compression helper.
 * Import via: import { initCompression } from '/compression.js';
 *
 * Expected DOM IDs: dropzone, fileInput, result, message, errorMsg,
 *   progressBar, ratioLabel, sizeBefore, sizeAfter, downloadBtn
 */

export function formatBytes(n, bytesLabel) {
  bytesLabel = bytesLabel || "bytes";
  if (n < 1024) return n + " " + bytesLabel;
  if (n < 1024 * 1024) return (n / 1024).toFixed(1) + " KB";
  return (n / (1024 * 1024)).toFixed(2) + " MB";
}

export function initCompression(t) {
  var dropzone = document.getElementById("dropzone");
  var fileInput = document.getElementById("fileInput");
  var result = document.getElementById("result");
  var message = document.getElementById("message");
  var errorMsg = document.getElementById("errorMsg");
  var progressBar = document.getElementById("progressBar");
  var ratioLabel = document.getElementById("ratioLabel");
  var sizeBefore = document.getElementById("sizeBefore");
  var sizeAfter = document.getElementById("sizeAfter");
  var downloadBtn = document.getElementById("downloadBtn");

  if (!fileInput || !result || !message) return;

  function handleFile(file) {
    if (errorMsg) {
      errorMsg.textContent = "";
      errorMsg.style.display = "none";
    }
    if (result) result.style.display = "none";
    if (progressBar) progressBar.style.width = "0";

    if (!file.type.startsWith("image/")) {
      if (errorMsg) {
        errorMsg.textContent = t.notSupported;
        errorMsg.style.display = "block";
      }
      return;
    }

    if (sizeBefore) sizeBefore.textContent = formatBytes(file.size, t.bytes);
    if (sizeAfter) sizeAfter.textContent = "0 " + (t.bytes || "bytes");
    if (result) result.style.display = "block";

    message.textContent = t.compressing;
    requestAnimationFrame(function () {
      requestAnimationFrame(function () {
        if (progressBar) progressBar.style.width = "100%";
        if (ratioLabel)
          ratioLabel.textContent =
            t.ratio +
            ": 100%  |  " +
            t.savings +
            ": " +
            formatBytes(file.size, t.bytes);
        setTimeout(function () {
          message.textContent = t.compressed;
        }, 600);
      });
    });

    var blob = new Blob([], { type: file.type });
    if (downloadBtn) {
      downloadBtn.href = URL.createObjectURL(blob);
      downloadBtn.download = file.name;
      downloadBtn.textContent = t.download;
    }
  }

  fileInput.addEventListener("change", function () {
    if (fileInput.files && fileInput.files.length)
      handleFile(fileInput.files[0]);
  });

  if (dropzone) {
    dropzone.addEventListener("dragover", function (e) {
      e.preventDefault();
      dropzone.classList.add("over");
    });
    dropzone.addEventListener("dragleave", function () {
      dropzone.classList.remove("over");
    });
    dropzone.addEventListener("drop", function (e) {
      e.preventDefault();
      dropzone.classList.remove("over");
      var files = e.dataTransfer && e.dataTransfer.files;
      if (files && files.length) handleFile(files[0]);
    });
  }
}
