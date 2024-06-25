// 'use client';
import { RiExternalLinkLine } from "@remixicon/react";
import { Card, Tab, TabGroup, TabList } from "@tremor/react";

const usage = [
  {
    id: 1,
    resource: "Requests per day",
    usage: "145",
    maximum: "1,000",
    href: "#",
  },
  {
    id: 2,
    resource: "Storage per month",
    usage: "1.1",
    maximum: "10 GB",
    href: "#",
  },
  {
    id: 3,
    resource: "Members",
    usage: "10",
    maximum: "25",
    href: "#",
  },
  {
    id: 4,
    resource: "Availability",
    usage: "95.1",
    maximum: "99.9%",
    href: "#",
  },
];

export default function Example() {
  return (
    <>
      <h3 className="text-tremor-title font-bold text-tremor-content-strong dark:text-dark-tremor-content-strong">
        Settings
      </h3>
      <p className="mt-2 text-tremor-default text-tremor-content dark:text-dark-tremor-content">
        Manage your personal details, workspace governance and notifications.
      </p>
      <TabGroup defaultIndex={1} className="mt-6">
        <TabList>
          <Tab>Account details</Tab>
          <Tab>Workspaces</Tab>
          <Tab>Billing</Tab>
        </TabList>
        {/* Content below only for demo purpose placed outside of <Tab> component. Add <TabPanels>, <TabPanel> to make it functional and to add content for other tabs */}
        <h4 className="mt-8 font-semibold text-tremor-content-strong dark:text-dark-tremor-content-strong">
          Workspace usage
        </h4>
        <p className="mt-1 text-tremor-default leading-6 text-tremor-content dark:text-dark-tremor-content">
          Overview of all activities of your workspace. Learn more about our{" "}
          <a
            href="#"
            className="inline-flex items-center gap-1 text-tremor-brand hover:underline hover:underline-offset-4 dark:text-dark-tremor-brand"
          >
            upgrade options
            <RiExternalLinkLine className="h-4 w-4" aria-hidden={true} />
          </a>
        </p>
        <div className="mt-6 grid grid-cols-1 gap-4 sm:max-w-3xl sm:grid-cols-2">
          {usage.map((item) => (
            <Card
              key={item.id}
              className="p-4 hover:bg-tremor-background-muted hover:dark:bg-dark-tremor-background-muted"
            >
              <p className="text-tremor-default text-tremor-content dark:text-dark-tremor-content">
                <a href={item.href} className="focus:outline-none">
                  {/* extend link to entire card */}
                  <span className="absolute inset-0" aria-hidden={true} />
                  {item.resource}
                </a>
              </p>
              <p className="mt-3 flex items-end">
                <span className="text-tremor-metric font-semibold text-tremor-content-strong dark:text-dark-tremor-content-strong">
                  {item.usage}
                </span>
                <span className="font-semibold text-tremor-content-subtle dark:text-dark-tremor-content-subtle">
                  /{item.maximum}
                </span>
              </p>
            </Card>
          ))}
        </div>
      </TabGroup>
    </>
  );
}
