import { TremorPlaceholder } from "@/components/ui/icons/TremorPlaceholder";

export default function Settings() {
  return (
    <>
      <div className="mt-4 sm:mt-6 lg:mt-10">
        <div className="my-40 flex w-full flex-col items-center justify-center">
          <TremorPlaceholder className="size-20 shrink-0" aria-hidden="true" />
          <h2 className="mt-6 text-lg font-semibold sm:text-xl">T.B.D.</h2>
          <p className="mt-3 max-w-md text-center text-gray-500">
            We are still working on this page. Check back soon!
          </p>
        </div>
      </div>
    </>
  );
}
