import { SnapCardExhibitModel } from "./Util";

class BtnShow {
  border: Boolean;
  exhibit: Boolean;
  copy: Boolean;
  send: Boolean;
  revise: Boolean;
  delete: Boolean;
  more: Boolean;
  snap_create: Boolean;
  
  constructor(in_config: {
    border?: Boolean,
    exhibit?: Boolean,
    copy?: Boolean,
    send?: Boolean,
    revise?: Boolean,
    delete?: Boolean,
    more?: Boolean,
    snap_create?: Boolean,
  }) {
    this.border = in_config.border === undefined ? false : in_config.border;
    this.exhibit = in_config.exhibit === undefined ? false : in_config.exhibit;
    this.copy = in_config.copy === undefined ? false : in_config.copy;
    this.send = in_config.send === undefined ? false : in_config.send;
    this.revise = in_config.revise === undefined ? false : in_config.revise;
    this.delete = in_config.delete === undefined ? false : in_config.delete;
    this.more = in_config.more === undefined ? false : in_config.more;
    this.snap_create = in_config.snap_create === undefined ? false : in_config.snap_create;
  }

  static useModelCreateShowConfig(model: SnapCardExhibitModel): BtnShow {
    let config = new BtnShow({});
    if (model === SnapCardExhibitModel.EXHIBIT_ON_SIDE) {
      config = new BtnShow({
        border: true,
        exhibit: true,
        copy: true,
        send: true,
        revise: false,
        delete: false,
        more: true,
        snap_create: false,
      });
    } else if (model === SnapCardExhibitModel.MANAGER_PANEL) {
      config = new BtnShow({
        border: true,
        exhibit: true,
        copy: true,
        send: false,
        revise: true,
        delete: true,
        more: false,
        snap_create: false,
      });
    } else if (model === SnapCardExhibitModel.CREATE_PANEL) {
      config = new BtnShow({
        border: false,
        exhibit: true,
        copy: true,
        send: false,
        revise: false,
        delete: false,
        more: false,
        snap_create: true,
      });
    } else if (model === SnapCardExhibitModel.MORE) {
      config = new BtnShow({
        border: false,
        exhibit: true,
        copy: true,
        send: true,
        revise: false,
        delete: false,
        more: false,
        snap_create: false,
      });
    } else if (model === SnapCardExhibitModel.SAVE) {
      config = new BtnShow({
        border: true,
        exhibit: true,
        copy: true,
        send: false,
        revise: false,
        delete: false,
        more: false,
        snap_create: false,
      });
    }
    return config;
  }
}

export default BtnShow;