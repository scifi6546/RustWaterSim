FROM rust:1-bullseye
RUN apt update -y && apt install pip zsh python3 python3-venv -y
RUN useradd -m -s /bin/zsh run
COPY exec.sh /home/run
COPY run.sh /home/run
RUN chmod +x /home/run/exec.sh
RUN chmod +x /home/run/run.sh
USER run
WORKDIR /home/run
COPY requirements.txt .
COPY test.py .

RUN ./exec.sh
#ENTRYPOINT [ "/bin/zsh" ]

#CMD ["ls","/bin"]
#CMD ["whoami"]
CMD ["./run.sh"]