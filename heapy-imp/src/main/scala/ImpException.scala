abstract class ImpException(msg: String, cause: Throwable = null) extends RuntimeException(msg, cause)

case class SegmentationFault(msg: String) extends ImpException(msg)

case class InvalidParameter(msg: String) extends ImpException(msg)