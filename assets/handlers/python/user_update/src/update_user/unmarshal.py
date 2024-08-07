from update_user.models import User


def unmarshal_user(json_data: bytes) -> User:
    return User.model_validate_json(json_data)
