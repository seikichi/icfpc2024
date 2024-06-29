-- CreateTable
CREATE TABLE "Experiment" (
    "id" SERIAL NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "commitId" VARCHAR(256) NOT NULL,
    "tag" VARCHAR(256) NOT NULL,
    "course" VARCHAR(256) NOT NULL,
    "levels" VARCHAR(256) NOT NULL,
    "args" VARCHAR(256) NOT NULL,

    CONSTRAINT "Experiment_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Run" (
    "id" SERIAL NOT NULL,
    "experimentId" INTEGER,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "commitId" VARCHAR(256) NOT NULL,
    "course" VARCHAR(256) NOT NULL,
    "level" INTEGER NOT NULL,
    "args" VARCHAR(256) NOT NULL,

    CONSTRAINT "Run_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "SuccessfulRun" (
    "id" SERIAL NOT NULL,
    "runId" INTEGER NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "elapsedSec" INTEGER NOT NULL,
    "score" BIGINT NOT NULL,

    CONSTRAINT "SuccessfulRun_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "FailedRun" (
    "id" SERIAL NOT NULL,
    "runId" INTEGER NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "elapsedSec" INTEGER NOT NULL,
    "error" TEXT NOT NULL,

    CONSTRAINT "FailedRun_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "SuccessfulRun_runId_key" ON "SuccessfulRun"("runId");

-- CreateIndex
CREATE UNIQUE INDEX "FailedRun_runId_key" ON "FailedRun"("runId");

-- AddForeignKey
ALTER TABLE "Run" ADD CONSTRAINT "Run_experimentId_fkey" FOREIGN KEY ("experimentId") REFERENCES "Experiment"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "SuccessfulRun" ADD CONSTRAINT "SuccessfulRun_runId_fkey" FOREIGN KEY ("runId") REFERENCES "Run"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "FailedRun" ADD CONSTRAINT "FailedRun_runId_fkey" FOREIGN KEY ("runId") REFERENCES "Run"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
