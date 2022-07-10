include $(TOPDIR)/rules.mk

PKG_NAME:=luci-app-geodata-reader
PKG_VERSION:=0.0.1
PKG_RELEASE:=1

PKG_LICENSE:=MPLv2
PKG_LICENSE_FILES:=LICENSE
PKG_MAINTAINER:=yichya <mail@yichya.dev>

PKG_BUILD_PARALLEL:=1

include $(INCLUDE_DIR)/package.mk

define Package/$(PKG_NAME)
	SECTION:=Custom
	CATEGORY:=Extra packages
	TITLE:=luci-app-geodata-reader
	DEPENDS:=+luci-base +xray-geodata
endef

define Package/$(PKG_NAME)/description
	read xray-geodata with web browser
endef

define Build/Compile
endef

define Package/$(PKG_NAME)/install
	$(INSTALL_DIR) $(1)/www
	$(INSTALL_DATA) ./docs/geodata_reader.js $(1)/www/geodata_reader.js
	$(INSTALL_DATA) ./docs/geodata_reader_bg.wasm $(1)/www/geodata_reader_bg.wasm
	$(LN) /usr/share/xray/geoip.dat $(1)/www/geoip.dat
	$(LN) /usr/share/xray/geosite.dat $(1)/www/geosite.dat
	$(INSTALL_DIR) $(1)/www/luci-static/resources/view
	$(INSTALL_DATA) ./luci/view.js $(1)/www/luci-static/resources/view/geodata_reader.js
	$(INSTALL_DIR) $(1)/usr/share/luci/menu.d
	$(INSTALL_DATA) ./luci/menu.json $(1)/usr/share/luci/menu.d/luci-app-geodata-reader.json
	$(INSTALL_DIR) $(1)/usr/share/rpcd/acl.d
	$(INSTALL_DATA) ./luci/acl.json $(1)/usr/share/rpcd/acl.d/luci-app-geodata-reader.json
endef

$(eval $(call BuildPackage,$(PKG_NAME)))
