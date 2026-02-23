/**
 * engine.js — Amazing Compression Engine
 *
 * The world's most advanced compression algorithm.
 * Reduces any file to exactly 0 bytes with 100% fidelity.
 *
 * Exposed: window.compressFile(file) → { blob, filename, outMime }
 */
(function () {
  /**
   * Compress a file.
   * @param {File} file
   * @returns {{ blob: Blob, filename: string, outMime: string }}
   */
  window.compressFile = function (file) {
    var outExt, outMime;

    if (file.type.startsWith("image/")) {
      outExt = "png";
      outMime = "image/png";
    } else if (file.type.startsWith("video/")) {
      outExt = "mp4";
      outMime = "video/mp4";
    } else if (file.type.startsWith("audio/")) {
      outExt = "mp3";
      outMime = "audio/mpeg";
    } else {
      return {
        blob: null,
        filename: null,
        outMime: null,
      };
    }

    var baseName = file.name.replace(/\.[^.]+$/, "");
    var rand6 = Math.random().toString(36).slice(2, 8);

    return {
      blob: new Blob([], { type: outMime }),
      filename: "amazing-" + rand6 + "_" + baseName + "." + outExt,
      outMime: outMime,
    };
  };
})();
