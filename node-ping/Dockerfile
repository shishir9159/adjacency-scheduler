FROM python:3.8
#COPY rootfs /
COPY . .
WORKDIR /app
RUN pip3 install --upgrade pip && pip3 install -r requirements.txt && apk add --no-cache fping
ENTRYPOINT ["python3", "/app/node-ping.py"]