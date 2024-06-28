import { Input } from "@/components/Input";
import { Label } from "@/components/Label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/Select";
import { Button } from "@/components/Button";

export default function Forms() {
  return (
    <>
      <section aria-labelledby="current-billing-cycle">
        <h1
          id="current-billing-cycle"
          className="scroll-mt-10 text-lg font-semibold text-gray-900 sm:text-xl dark:text-gray-50"
        >
          ICFPC 2024
        </h1>
        <div className="mt-4 sm:mt-6 lg:mt-10">
          <div className="my-40 flex w-full flex-col items-center justify-center">
            <h2 className="mt-6 text-lg font-semibold sm:text-xl">Form</h2>

            <form>
              <div className="flex flex-col gap-y-1 sm:max-w-2xl">
                <div className="mt-4 grid grid-cols-2 gap-4">
                  <div>
                    <Label htmlFor="workspace-name" className="font-medium">
                      Workspace name
                    </Label>
                    <Input
                      id="workspace-name"
                      name="workspace-name"
                      placeholder="my_workspace"
                      className="mt-2"
                    />
                  </div>
                  <div>
                    <Label htmlFor="starter-kit" className="font-medium">
                      Starter kit
                    </Label>
                    <Select defaultValue="empty-workspace">
                      <SelectTrigger
                        id="starter-kit"
                        name="starter-kit"
                        className="mt-2"
                      >
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="empty-workspace">
                          None - Empty workspace
                        </SelectItem>
                        <SelectItem value="commerce-analytics">
                          Commerce analytics
                        </SelectItem>
                        <SelectItem value="product-analytics">
                          Product analytics
                        </SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div className="col-span-full">
                    <Label htmlFor="database-region" className="font-medium">
                      Database region
                    </Label>
                    <Select defaultValue="europe-west-01">
                      <SelectTrigger
                        id="database-region"
                        name="database-region"
                        className="mt-2"
                      >
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="europe-west-01">
                          europe-west-01
                        </SelectItem>
                        <SelectItem value="us-east-02">us-east-02</SelectItem>
                        <SelectItem value="us-west-01">us-west-01</SelectItem>
                      </SelectContent>
                    </Select>
                    <p className="mt-2 text-xs text-gray-500">
                      For best performance, choose a region closest to your
                      application.
                    </p>
                  </div>
                </div>
              </div>
              <div className="flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 mt-4">
                <Button
                  type="button"
                  className="mt-2 w-full sm:mt-0 sm:w-fit"
                  variant="secondary"
                >
                  Go back
                </Button>
                <Button type="button" className="w-full sm:w-fit">
                  Add workspace
                </Button>
              </div>
            </form>
          </div>
        </div>
      </section>
    </>
  );
}
