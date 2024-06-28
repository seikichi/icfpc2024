"use client";

import { siteConfig } from "@/app/siteConfig";
import {
  RiHome2Line,
  // RiTableLine,
  // RiBarChartLine,
  // RiCodeLine,
  // RiDatabaseLine,
  // RiLinkM,
  // RiInputField,
} from "@remixicon/react";

export const navigation = [
  { name: "Overview", href: siteConfig.baseLinks.overview, icon: RiHome2Line },
  // { name: "Details", href: siteConfig.baseLinks.details, icon: RiListCheck },
  //   {
  //     name: "Settings",
  //     href: siteConfig.baseLinks.settings,
  //     icon: RiSettings5Line,
  //   },
  // { name: "Tables", href: siteConfig.baseLinks.tables, icon: RiTableLine },
  // { name: "Charts", href: siteConfig.baseLinks.charts, icon: RiBarChartLine },
  // { name: "WASM", href: siteConfig.baseLinks.wasm, icon: RiCodeLine },
  // { name: "Database", href: siteConfig.baseLinks.db, icon: RiDatabaseLine },
  // { name: "Forms", href: siteConfig.baseLinks.forms, icon: RiInputField },
] as const;

export const shortcuts = [
  // {
  //   name: "Run WASM",
  //   href: siteConfig.baseLinks.wasm,
  //   icon: RiLinkM,
  // },
] as const;
