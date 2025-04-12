FROM python:3.8
COPY . .
RUN pip3 install --upgrade pip && pip3 install -r requirements.txt && apk add --no-cache fping
WORKDIR /app
ENTRYPOINT ["python3", "/app/node-ping.py"]