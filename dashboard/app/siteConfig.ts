export const siteConfig = {
  name: "Dashboard",
  url: "https://kmc-ob-icfpc2024.vercel.app/",
  description: "ICFPC 2024",
  baseLinks: {
    home: "/",
    overview: "/overview",
    details: "/details",
    settings: "/settings",
    tables: "/tables",
    charts: "/charts",
    db: "/db",
    wasm: "/wasm",
  },
};

export type siteConfig = typeof siteConfig;
