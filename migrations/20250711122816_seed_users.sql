-- Add migration script here
INSERT INTO newsletter_writers (user_id, username, salted_password)
VALUES (
    '4285294e-5251-4328-996d-5044ef277535',
    'admin',
    '$argon2id$v=19$m=19456,t=2,p=1$cZcQp1wUQ+IUagauInfojA$XOL0Re0wg5ypW0JdeO5GPUe+q6/rIDEF8HYp1enEm0I'
)