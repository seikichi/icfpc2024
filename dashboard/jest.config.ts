import nextJest from "next/jest";

const createJestConfig = nextJest({ dir: "./" });

const customJestConfig = {};

export default createJestConfig(customJestConfig);
