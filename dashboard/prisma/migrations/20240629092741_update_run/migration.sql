/*
  Warnings:

  - You are about to drop the `FailedRun` table. If the table is not empty, all the data it contains will be lost.
  - You are about to drop the `SuccessfulRun` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "FailedRun" DROP CONSTRAINT "FailedRun_runId_fkey";

-- DropForeignKey
ALTER TABLE "SuccessfulRun" DROP CONSTRAINT "SuccessfulRun_runId_fkey";

-- AlterTable
ALTER TABLE "Run" ADD COLUMN     "elapsedSec" INTEGER,
ADD COLUMN     "error" TEXT,
ADD COLUMN     "score" BIGINT;

-- DropTable
DROP TABLE "FailedRun";

-- DropTable
DROP TABLE "SuccessfulRun";
