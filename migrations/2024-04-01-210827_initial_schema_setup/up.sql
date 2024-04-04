-- Your SQL goes here

CREATE TABLE "proposals"(
	"id" UUID NOT NULL PRIMARY KEY,
	"maker_id" UUID NOT NULL,
	"participants" UUID[] NOT NULL,
	"payload_hint" VARCHAR NOT NULL,
	"info" TEXT NOT NULL
);

CREATE TABLE "considerations"(
	"id" UUID NOT NULL PRIMARY KEY,
	"proposal_id" UUID NOT NULL,
	"taker_id" UUID NOT NULL,
	"payload_hint" VARCHAR NOT NULL,
	"info" TEXT NOT NULL,
	FOREIGN KEY ("proposal_id") REFERENCES "proposals"("id")
);

CREATE TABLE "agreements"(
	"id" UUID NOT NULL PRIMARY KEY,
	"consideration_id" UUID NOT NULL,
	"maker_id" UUID NOT NULL,
	"taker_id" UUID NOT NULL,
	"payload_hint" VARCHAR NOT NULL,
	"info" TEXT NOT NULL,
	FOREIGN KEY ("consideration_id") REFERENCES "considerations"("id")
);

