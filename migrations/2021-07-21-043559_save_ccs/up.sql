-- Your SQL goes here
ALTER TABLE "public"."extra_info"
    ALTER COLUMN "id_request" DROP DEFAULT,
    ALTER COLUMN "id_request" ADD GENERATED BY DEFAULT AS IDENTITY;