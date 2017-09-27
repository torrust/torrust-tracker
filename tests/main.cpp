#include <gtest/gtest.h>
#include "../src/tools.h"
#include "../src/db/driver_sqlite.hpp"

TEST(Utility, SanityCheck) {
    const uint32_t MAGIC = 0xDEADBEEF;
    const unsigned char MAGIC_BYTES[4] = {0xEF, 0xBE, 0xAD, 0xDE};
    ASSERT_TRUE(memcmp(&MAGIC, MAGIC_BYTES, 4) == 0);
}

TEST(Utility, CheckMTON) {
    EXPECT_EQ(m_hton16(0xDEAD), 0xADDE);
    EXPECT_EQ(m_hton32(0xDEADBEEF), 0xEFBEADDE);
    EXPECT_EQ(m_hton64(0xDEADBEEFA1B2C3E4), 0xE4C3B2A1EFBEADDE);
}

TEST(Utility, HashToHexStr) {
    const char EXPECTED_OUTPUT[] = "c670606edd22fd0e3b432c977559a687cc5d9bd2";
    const unsigned char DATA[20] = {198, 112, 96, 110, 221, 34, 253, 14, 59, 67, 44, 151, 117, 89, 166, 135, 204, 93, 155, 210};

    char OUTPUT_BUFFER[41] = {0};
    to_hex_str(DATA, OUTPUT_BUFFER);

    ASSERT_EQ(std::string(EXPECTED_OUTPUT), OUTPUT_BUFFER);
}

class SQLiteDriverTest:
        public ::testing::Test {
protected:
    SQLiteDriverTest(): va_map(), driver(nullptr) {
        va_map.insert(std::pair<std::string, boost::program_options::variable_value>("db.param", boost::program_options::variable_value(std::string(":memory:"), true)));
    }

    virtual void SetUp() {
        if (nullptr == driver) {
            driver = new UDPT::Data::SQLite3Driver(va_map, false);
        }
    }

    virtual void TearDown() {
        if (nullptr != driver) {
            delete driver;
            driver = nullptr;
        }
    }

    UDPT::Data::SQLite3Driver *driver;
    boost::program_options::variables_map va_map;
};


int main(int argc, char *argv[]) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
