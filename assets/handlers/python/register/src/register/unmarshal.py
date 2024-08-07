from register.models import CreateUser


def unmarshal_user(json_data: bytes) -> CreateUser:
    return CreateUser.model_validate_json(json_data)
