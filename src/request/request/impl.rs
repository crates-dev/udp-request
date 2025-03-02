use crate::*;

impl UdpRequest {
    #[inline]
    fn send_request(
        &mut self,
        socket: &mut UdpSocket,
        data: &[u8],
    ) -> Result<BoxResponseTrait, Error> {
        socket
            .send(data)
            .map_err(|err| Error::SendResponseError(err.to_string()))?;
        self.read_response(socket)
    }

    #[inline]
    fn read_response(&mut self, socket: &mut UdpSocket) -> Result<BoxResponseTrait, Error> {
        let cfg_buffer_size: usize = self
            .get_config()
            .read()
            .map_or(DEFAULT_BUFFER_SIZE, |data| data.buffer_size);
        let mut tmp_buf: Vec<u8> = vec![0u8; cfg_buffer_size];
        let mut response_bytes: Vec<u8> = Vec::with_capacity(cfg_buffer_size);
        if let Ok(n) = socket.recv(&mut tmp_buf) {
            response_bytes.extend_from_slice(&tmp_buf[..n]);
        }
        self.response = Arc::new(RwLock::new(<UdpResponseBinary as ResponseTrait>::from(
            &response_bytes,
        )));
        return Ok(Box::new(
            self.response.read().map_or(Vec::new(), |data| data.clone()),
        ));
    }

    #[inline]
    fn get_connection_socket(&self, host: String, port: usize) -> Result<UdpSocket, Error> {
        let host_port: String = format!("{}:{}", host.clone(), port);
        let cfg_timeout: u64 = self
            .get_config()
            .read()
            .map_or(DEFAULT_TIMEOUT, |data| data.timeout);
        let timeout: Duration = Duration::from_millis(cfg_timeout);
        let socket: UdpSocket =
            UdpSocket::bind("0.0.0.0:0").map_err(|_| Error::UdpSocketCreateError)?;
        socket
            .connect(host_port)
            .map_err(|_| Error::UdpSocketConnectError)?;
        socket
            .set_read_timeout(Some(timeout))
            .map_err(|_| Error::SetReadTimeoutError)?;
        socket
            .set_write_timeout(Some(timeout))
            .map_err(|_| Error::SetWriteTimeoutError)?;
        let socket_result: Result<UdpSocket, Error> = Ok(socket);
        socket_result
    }
}

impl RequestTrait for UdpRequest {
    type RequestResult = RequestResult;

    #[inline]
    fn send(&mut self, data: &[u8]) -> Self::RequestResult {
        let cfg_timeout: Config = self
            .get_config()
            .read()
            .map_or(Config::default(), |data| data.clone());
        let host: String = cfg_timeout.get_host().clone();
        let port: usize = cfg_timeout.get_port().clone();
        let mut socket: UdpSocket = self.get_connection_socket(host, port)?;
        let res: Result<BoxResponseTrait, Error> = self.send_request(&mut socket, data);
        res
    }
}

impl Default for UdpRequest {
    #[inline]
    fn default() -> Self {
        Self {
            config: Arc::new(RwLock::new(Config::default())),
            response: Arc::new(RwLock::new(UdpResponseBinary::default())),
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}
