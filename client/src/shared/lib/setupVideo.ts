import Hls from "hls.js";

export const setupVideo = (
  videoSrc: string,
  videoEl: HTMLVideoElement | null
) => {
  if (videoEl && Hls.isSupported()) {
    const hls = new Hls();
    hls.loadSource(
      `${import.meta.env.VITE_SERVER_URL}/get_vod?vod_url=${videoSrc}`
    );
    hls.attachMedia(videoEl);
  } else if (videoEl?.canPlayType("application/vnd.apple.mpegurl")) {
    videoEl.src = `${
      import.meta.env.VITE_SERVER_URL
    }/get_vod?vod_url=${videoSrc}`;
  }
};
