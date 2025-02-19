FROM python:3.10-slim

ENV VENV_PATH=/opt/venv
RUN python -m venv ${VENV_PATH}
ENV PATH="${VENV_PATH}/bin:$PATH"

WORKDIR /seed
COPY pyproject.toml pyproject.toml
COPY src src
RUN pip install --upgrade pip && \
    pip install --no-cache-dir -e .
ENTRYPOINT [ "python", "src/seed_invoice/main.py" ]
