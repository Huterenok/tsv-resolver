import { ChangeEvent, FC, useRef, useState } from "react";

import { setupVideo } from "shared/lib";
import { Button, Input } from "shared/ui";

import styles from "./Main.module.css";

export const Main: FC = () => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [videoSrc, setVideoSrc] = useState<string>("");

  const onInput = (e: ChangeEvent<HTMLInputElement>) => {
    setVideoSrc(e.target.value);
  };

  const onGetVod = () => {
    setupVideo(videoSrc, videoRef.current);
  };

  return (
    <main className={styles.wrapper}>
      <video
        controls
        className={styles.video}
        ref={videoRef}
      />
      <Input
        value={videoSrc}
        onChange={onInput}
        placeholder="Link to twitch vod"
        className={styles.input}
      />
      <Button onClick={onGetVod}>Find vod</Button>
    </main>
  );
};
