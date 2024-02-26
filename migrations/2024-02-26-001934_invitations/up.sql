-- Your SQL goes here
CREATE TABLE "invitations" (
    invitation_token UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    used BOOLEAN NOT NULL DEFAULT false,
    expires_at TIMESTAMP NOT NULL
);