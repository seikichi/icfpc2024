import type { Metadata } from "next";

import { Inter } from "next/font/google";

const inter = Inter({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-inter",
});

export const metadata: Metadata = {
  metadataBase: new URL("https://kmc-ob-icfpc2024.vercel.app/"),
  title: "ICFPC 2024",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className}`}>
        <nav>
          <ul>
            <li>
              <a href="/">Top</a>
            </li>
            <li>
              <a href="/communicate">Communicate</a>
            </li>
            <li>
              <a href="/browse/index">Browse Galaxy</a>
            </li>
            <li>
              <a href="/experiments">Experiments</a>
            </li>
          </ul>
        </nav>
        <main>{children}</main>
      </body>
    </html>
  );
}
