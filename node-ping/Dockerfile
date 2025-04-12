FROM 3.8-alpine3.20
WORKDIR /app
COPY . .
RUN pip3 install --upgrade pip && pip3 install -r requirements.txt && apk add --no-cache fping
ENTRYPOINT ["python3", "/app/node-ping.py"]