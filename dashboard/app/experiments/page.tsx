import ExperimentResults from "./results";
import SubmitForm from "./submit";

export const dynamic = "force-dynamic";

export default function Page() {
  return (
    <>
      <h1>Experiments</h1>
      <section>
        <SubmitForm />
      </section>
      <section>
        <ExperimentResults />
      </section>
    </>
  );
}
