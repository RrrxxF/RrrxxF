// Thumbnails load only when cards enter view; SVG source is never rendered in the WebView.
import { useEffect, useRef, useState } from "react";
import { assetApi, type Asset } from "../../services/assets";

export function AssetThumbnail({ asset }: { asset: Asset }) {
  const element = useRef<HTMLDivElement>(null);
  const [visible, setVisible] = useState(false);
  const [url, setUrl] = useState<string | null>(null);

  useEffect(() => {
    const node = element.current;
    if (!node) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) setVisible(true);
      },
      { rootMargin: "180px" },
    );
    observer.observe(node);
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    if (!visible || !asset.hasThumbnail) return;
    let live = true;
    let objectUrl: string | null = null;
    assetApi
      .thumbnail(asset.id)
      .then((bytes) => {
        if (!live || !bytes) return;
        objectUrl = URL.createObjectURL(
          new Blob([new Uint8Array(bytes)], { type: "image/webp" }),
        );
        setUrl(objectUrl);
      })
      .catch(() => undefined);
    return () => {
      live = false;
      if (objectUrl) URL.revokeObjectURL(objectUrl);
    };
  }, [asset.id, asset.hasThumbnail, visible]);

  return (
    <div className="asset-thumbnail" ref={element}>
      {url ? (
        <img src={url} alt="" loading="lazy" />
      ) : (
        <span aria-hidden="true">
          {asset.mime === "image/svg+xml" ? "◇" : "▧"}
        </span>
      )}
    </div>
  );
}
