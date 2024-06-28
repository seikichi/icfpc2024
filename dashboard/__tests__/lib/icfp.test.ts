import * as icfp from "@/lib/icfp";

test("encodeString('Hello, Wrold!')", () => {
  expect(icfp.encodeString("Hello World!")).toStrictEqual("SB%,,/}Q/2,$_");
});

test("decodeString('SB%,,/}Q/2,$_')", () => {
  expect(icfp.decodeString("SB%,,/}Q/2,$_")).toStrictEqual("Hello World!");
});
