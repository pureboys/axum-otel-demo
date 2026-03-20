#!/bin/bash
#===============================================
# Axum 项目压测脚本
#===============================================

set -e

# 配置
HOST="http://127.0.0.1:8000"
DURATION=30        # 压测持续时间（秒）
CONNECTIONS=100    # 并发连接数
THREADS=4          # 线程数

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}    Axum 项目压测脚本${NC}"
echo -e "${GREEN}========================================${NC}"

# 检测服务器是否运行
echo -e "\n${YELLOW}[1/4] 检测服务器...${NC}"
if curl -s --connect-timeout 2 "${HOST}/api/common/captcha" > /dev/null 2>&1; then
    echo -e "${GREEN}✓ 服务器运行正常${NC}"
else
    echo -e "${RED}✗ 服务器未运行，请先启动: cargo run --bin axum-otel-demo -- --env dev${NC}"
    exit 1
fi

# 检测压测工具
echo -e "\n${YELLOW}[2/4] 检测压测工具...${NC}"

BENCH_TOOL=""
if command -v ab &> /dev/null; then
    BENCH_TOOL="ab"
    echo -e "${GREEN}✓ 找到 ab (Apache Bench)${NC}"
elif command -v hey &> /dev/null; then
    BENCH_TOOL="hey"
    echo -e "${GREEN}✓ 找到 hey${NC}"
elif command -v wrk &> /dev/null; then
    BENCH_TOOL="wrk"
    echo -e "${GREEN}✓ 找到 wrk${NC}"
else
    echo -e "${RED}✗ 未找到压测工具，请安装以下任一工具:${NC}"
    echo "  - ab: brew install httpd (macOS) 或 apt install apache2-utils (Linux)"
    echo "  - hey: go install github.com/rakyll/hey@latest"
    echo "  - wrk: brew install wrk (macOS) 或 apt install wrk (Linux)"
    exit 1
fi

# 显示测试接口
echo -e "\n${YELLOW}[3/4] 压测配置...${NC}"
echo "  目标主机: ${HOST}"
echo "  压测工具: ${BENCH_TOOL}"
echo "  持续时间: ${DURATION}s"

case $BENCH_TOOL in
    ab)
        echo "  并发数:  ${CONNECTIONS}"
        echo "  总请求:  ${CONNECTIONS} (串行)"
        ;;
    hey)
        echo "  并发数:  ${CONNECTIONS}"
        echo "  请求数:  1000"
        ;;
    wrk)
        echo "  线程数:  ${THREADS}"
        echo "  连接数:  ${CONNECTIONS}"
        ;;
esac

# 执行压测
echo -e "\n${YELLOW}[4/4] 开始压测...${NC}"
echo -e "${GREEN}----------------------------------------${NC}"

case $BENCH_TOOL in
    ab)
        echo "压测接口: GET /api/common/captcha"
        ab -n 1000 -c ${CONNECTIONS} -t ${DURATION} "${HOST}/api/common/captcha"
        ;;

    hey)
        echo "压测接口: GET /api/common/captcha"
        hey -n 1000 -c ${CONNECTIONS} -t ${DURATION} "${HOST}/api/common/captcha"
        ;;

    wrk)
        echo "压测接口: GET /api/common/captcha"
        wrk -t${THREADS} -c${CONNECTIONS} -d${DURATION}s "${HOST}/api/common/captcha"
        ;;
esac

echo -e "${GREEN}----------------------------------------${NC}"
echo -e "${GREEN}压测完成${NC}"
