-- CreateIndex
CREATE INDEX "Experiment_createdAt_idx" ON "Experiment"("createdAt");

-- CreateIndex
CREATE INDEX "Run_course_level_createdAt_idx" ON "Run"("course", "level", "createdAt");

-- CreateIndex
CREATE INDEX "Run_course_level_score_idx" ON "Run"("course", "level", "score");
